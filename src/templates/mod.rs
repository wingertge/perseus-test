use std::rc::Rc;

use sycamore::reactive::{create_signal, ReadSignal, Scope, Signal};

pub mod counter;
pub mod hello;
pub mod index;

#[macro_export]
macro_rules! create_refetch {
    ($cx: expr, $signal: expr, $fetcher: expr) => {
        move || {
            #[cfg(target_arch = "wasm32")]
            ::perseus::spawn_local_scoped($cx, async {
                let res = ($fetcher)().await;
                $signal.set(res);
            })
        }
    };
}

#[macro_export]
macro_rules! sided {
    ($client: expr, $server: expr) => {{
        #[cfg(target_arch = "wasm32")]
        {
            $client
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            $server
        }
    }};
}

#[macro_export]
macro_rules! create_query {
    ($cx: expr, $data: expr, $fetcher: expr) => {{
        let fetching = create_signal($cx, false);
        let refetch = move || {
            #[cfg(target_arch = "wasm32")]
            ::perseus::spawn_local_scoped($cx, async {
                fetching.set(true);
                let res = $fetcher.await;
                $data.set(res);
                fetching.set(false);
            })
        };
        (fetching, refetch)
    }};
}

#[macro_export]
macro_rules! use_query {
    ($cx: expr, $fetcher: expr) => {{
        async {
            let data = create_signal($cx, $fetcher.await);
            let fetching = create_signal($cx, false);
            let refetch = move || {
                ::perseus::spawn_local_scoped($cx, async {
                    fetching.set(true);
                    let res = $fetcher.await;
                    data.set(res);
                    fetching.set(false);
                })
            };
            (data, fetching, refetch)
        }
    }};
}

#[macro_export]
macro_rules! use_query_res {
    ($cx: expr, $fetcher: expr) => {{
        async {
            let data = $crate::templates::create_res_signal($cx, $fetcher.await);
            let fetching = create_signal($cx, false);
            let refetch = move || {
                ::perseus::spawn_local_scoped($cx, async {
                    fetching.set(true);
                    let res = $fetcher.await;
                    data.set_res(res);
                    fetching.set(false);
                })
            };
            (data, fetching, refetch)
        }
    }};
}

#[allow(unused)]
pub fn create_res_signal<T, E>(cx: Scope, value: Result<T, E>) -> &Signal<Result<Rc<T>, Rc<E>>> {
    create_signal(cx, value.map(Rc::new).map_err(Rc::new))
}

#[allow(unused)]
pub fn create_opt_signal<T>(cx: Scope, value: Option<T>) -> &Signal<Option<Rc<T>>> {
    create_signal(cx, value.map(Rc::new))
}

pub trait SignalResultExt<T, E> {
    fn set_res(&self, value: Result<T, E>);
}

pub trait ReadSignalResultExt<T, E> {
    fn get_res(&self) -> Result<Rc<T>, Rc<E>>;
}

pub trait SignalOptionExt<T> {
    fn set_opt(&self, value: Option<T>);
}

pub trait ReadSignalOptionExt<T> {
    fn get_opt(&self) -> Option<Rc<T>>;
}

impl<T, E> SignalResultExt<T, E> for Signal<Result<Rc<T>, Rc<E>>> {
    fn set_res(&self, value: Result<T, E>) {
        self.set(value.map(Rc::new).map_err(Rc::new));
    }
}

impl<T, E> ReadSignalResultExt<T, E> for ReadSignal<Result<Rc<T>, Rc<E>>> {
    fn get_res(&self) -> Result<Rc<T>, Rc<E>> {
        match self.get().as_ref() {
            Ok(value) => Ok(value.clone()),
            Err(err) => Err(err.clone()),
        }
    }
}

impl<T> SignalOptionExt<T> for Signal<Option<Rc<T>>> {
    fn set_opt(&self, value: Option<T>) {
        self.set(value.map(Rc::new));
    }
}

impl<T> ReadSignalOptionExt<T> for ReadSignal<Option<Rc<T>>> {
    fn get_opt(&self) -> Option<Rc<T>> {
        self.get().as_ref().as_ref().map(Clone::clone)
    }
}
