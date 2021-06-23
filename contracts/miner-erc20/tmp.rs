#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use ink_prelude::string::String;
pub use erc20_ownable::Erc20Minable;
mod erc20_ownable {
    impl ::ink_lang::ContractEnv for Erc20Minable {
        type Env = ::ink_env::DefaultEnvironment;
    }
    type Environment = <Erc20Minable as ::ink_lang::ContractEnv>::Env;
    type AccountId =
        <<Erc20Minable as ::ink_lang::ContractEnv>::Env as ::ink_env::Environment>::AccountId;
    type Balance =
        <<Erc20Minable as ::ink_lang::ContractEnv>::Env as ::ink_env::Environment>::Balance;
    type Hash = <<Erc20Minable as ::ink_lang::ContractEnv>::Env as ::ink_env::Environment>::Hash;
    type Timestamp =
        <<Erc20Minable as ::ink_lang::ContractEnv>::Env as ::ink_env::Environment>::Timestamp;
    type BlockNumber =
        <<Erc20Minable as ::ink_lang::ContractEnv>::Env as ::ink_env::Environment>::BlockNumber;
    #[cfg(not(feature = "ink-as-dependency"))]
    const _: () = {
        impl<'a> ::ink_lang::Env for &'a Erc20Minable {
            type EnvAccess =
                ::ink_lang::EnvAccess<'a, <Erc20Minable as ::ink_lang::ContractEnv>::Env>;
            fn env(self) -> Self::EnvAccess {
                Default::default()
            }
        }
        impl<'a> ::ink_lang::StaticEnv for Erc20Minable {
            type EnvAccess =
                ::ink_lang::EnvAccess<'static, <Erc20Minable as ::ink_lang::ContractEnv>::Env>;
            fn env() -> Self::EnvAccess {
                Default::default()
            }
        }
    };
    #[cfg(not(feature = "ink-as-dependency"))]
    /// A simple ERC-20 contract.
    pub struct Erc20Minable {
        erc20: erc20::Data<Erc20Minable>,
    }
    const _: () = {
        impl ::ink_storage::traits::SpreadLayout for Erc20Minable {
            #[allow(unused_comparisons)]
            const FOOTPRINT : u64 = [(0u64 + < erc20 :: Data < Erc20Minable > as :: ink_storage :: traits :: SpreadLayout > :: FOOTPRINT) , 0u64] [((0u64 + < erc20 :: Data < Erc20Minable > as :: ink_storage :: traits :: SpreadLayout > :: FOOTPRINT) < 0u64) as usize] ;
            const REQUIRES_DEEP_CLEAN_UP : bool = (false || (false || < erc20 :: Data < Erc20Minable > as :: ink_storage :: traits :: SpreadLayout > :: REQUIRES_DEEP_CLEAN_UP)) ;
            fn pull_spread(__key_ptr: &mut ::ink_storage::traits::KeyPtr) -> Self {
                Erc20Minable { erc20 : < erc20 :: Data < Erc20Minable > as :: ink_storage :: traits :: SpreadLayout > :: pull_spread (__key_ptr) , }
            }
            fn push_spread(&self, __key_ptr: &mut ::ink_storage::traits::KeyPtr) {
                match self {
                    Erc20Minable { erc20: __binding_0 } => {
                        ::ink_storage::traits::SpreadLayout::push_spread(__binding_0, __key_ptr);
                    }
                }
            }
            fn clear_spread(&self, __key_ptr: &mut ::ink_storage::traits::KeyPtr) {
                match self {
                    Erc20Minable { erc20: __binding_0 } => {
                        ::ink_storage::traits::SpreadLayout::clear_spread(__binding_0, __key_ptr);
                    }
                }
            }
        }
    };
    const _: () = {
        impl ::ink_storage::traits::StorageLayout for Erc20Minable {
            fn layout(
                __key_ptr: &mut ::ink_storage::traits::KeyPtr,
            ) -> ::ink_metadata::layout::Layout {
                ::ink_metadata::layout::Layout::Struct(::ink_metadata::layout::StructLayout::new(
                    <[_]>::into_vec(box [::ink_metadata::layout::FieldLayout::new(
                        Some("erc20"),
                        <erc20::Data<Erc20Minable> as ::ink_storage::traits::StorageLayout>::layout(
                            __key_ptr,
                        ),
                    )]),
                ))
            }
        }
    };
    #[cfg(not(feature = "ink-as-dependency"))]
    const _: () = {
        #[allow(unused_imports)]
        use ::ink_lang::{Env as _, StaticEnv as _};
        use ::ink_lang::EmitEvent as _;
    };
    const _: () = {
        #[cfg(not(feature = "ink-as-dependency"))]
        impl<'a> ::ink_lang::EmitEvent<Erc20Minable> for ::ink_lang::EnvAccess<'a, Environment> {
            fn emit_event<E>(self, event: E)
            where
                E: Into<<Erc20Minable as ::ink_lang::BaseEvent>::Type>,
            {
                ::ink_env::emit_event::<Environment, <Erc20Minable as ::ink_lang::BaseEvent>::Type>(
                    event.into(),
                );
            }
        }
    };
    #[cfg(not(feature = "ink-as-dependency"))]
    pub enum __ink_EventBase {
        Transfer(Transfer),
        Approval(Approval),
    }
    const _: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate scale as _parity_scale_codec;
        impl _parity_scale_codec::Encode for __ink_EventBase {
            fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output + ?Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                match *self {
                    __ink_EventBase::Transfer(ref aa) => {
                        __codec_dest_edqy.push_byte(0usize as ::core::primitive::u8);
                        _parity_scale_codec::Encode::encode_to(aa, __codec_dest_edqy);
                    }
                    __ink_EventBase::Approval(ref aa) => {
                        __codec_dest_edqy.push_byte(1usize as ::core::primitive::u8);
                        _parity_scale_codec::Encode::encode_to(aa, __codec_dest_edqy);
                    }
                    _ => (),
                }
            }
        }
        impl _parity_scale_codec::EncodeLike for __ink_EventBase {}
    };
    const _: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate scale as _parity_scale_codec;
        impl _parity_scale_codec::Decode for __ink_EventBase {
            fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, _parity_scale_codec::Error> {
                match __codec_input_edqy.read_byte().map_err(|e| {
                    e.chain("Could not decode `__ink_EventBase`, failed to read variant byte")
                })? {
                    __codec_x_edqy if __codec_x_edqy == 0usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(__ink_EventBase::Transfer({
                            let __codec_res_edqy =
                                <Transfer as _parity_scale_codec::Decode>::decode(
                                    __codec_input_edqy,
                                );
                            match __codec_res_edqy {
                                ::core::result::Result::Err(e) => {
                                    return ::core::result::Result::Err(
                                        e.chain("Could not decode `__ink_EventBase::Transfer.0`"),
                                    )
                                }
                                ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        }))
                    }
                    __codec_x_edqy if __codec_x_edqy == 1usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(__ink_EventBase::Approval({
                            let __codec_res_edqy =
                                <Approval as _parity_scale_codec::Decode>::decode(
                                    __codec_input_edqy,
                                );
                            match __codec_res_edqy {
                                ::core::result::Result::Err(e) => {
                                    return ::core::result::Result::Err(
                                        e.chain("Could not decode `__ink_EventBase::Approval.0`"),
                                    )
                                }
                                ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        }))
                    }
                    _ => ::core::result::Result::Err(
                        "Could not decode `__ink_EventBase`, variant doesn\'t exist".into(),
                    ),
                }
            }
        }
    };
    #[cfg(not(feature = "ink-as-dependency"))]
    const _: () = {
        impl ::ink_lang::BaseEvent for Erc20Minable {
            type Type = __ink_EventBase;
        }
    };
    #[cfg(not(feature = "ink-as-dependency"))]
    const _: () = {
        impl From<Transfer> for __ink_EventBase {
            fn from(event: Transfer) -> Self {
                Self::Transfer(event)
            }
        }
    };
    #[cfg(not(feature = "ink-as-dependency"))]
    const _: () = {
        impl From<Approval> for __ink_EventBase {
            fn from(event: Approval) -> Self {
                Self::Approval(event)
            }
        }
    };
    const _: () = {
        pub enum __ink_UndefinedAmountOfTopics {}
        impl ::ink_env::topics::EventTopicsAmount for __ink_UndefinedAmountOfTopics {
            const AMOUNT: usize = 0;
        }
        #[cfg(not(feature = "ink-as-dependency"))]
        impl ::ink_env::Topics for __ink_EventBase {
            type RemainingTopics = __ink_UndefinedAmountOfTopics;
            fn topics<E, B>(
                &self,
                builder: ::ink_env::topics::TopicsBuilder<::ink_env::topics::state::Uninit, E, B>,
            ) -> <B as ::ink_env::topics::TopicsBuilderBackend<E>>::Output
            where
                E: ::ink_env::Environment,
                B: ::ink_env::topics::TopicsBuilderBackend<E>,
            {
                match self {
                    Self::Transfer(event) => {
                        <Transfer as ::ink_env::Topics>::topics::<E, B>(event, builder)
                    }
                    Self::Approval(event) => {
                        <Approval as ::ink_env::Topics>::topics::<E, B>(event, builder)
                    }
                }
            }
        }
    };
    #[cfg(not(feature = "ink-as-dependency"))]
    const _: () = {
        #[allow(non_camel_case_types)]
        pub enum __ink_CheckSatisfied {}
        pub enum EventTopicsWithinBounds {}
        impl ::ink_lang::True for __ink_CheckSatisfied {}
        #[doc(hidden)]
        pub trait CompliesWithTopicLimit {}
        impl CompliesWithTopicLimit for __ink_CheckSatisfied {}
        #[allow(non_camel_case_types)]
        pub trait __ink_RenameBool {
            type Type;
        }
        impl __ink_RenameBool for [(); true as usize] {
            type Type = __ink_CheckSatisfied;
        }
        impl __ink_RenameBool for [(); false as usize] {
            type Type = Transfer;
        }
        #[allow(non_upper_case_globals)]
        const __ink_MAX_EVENT_TOPICS : usize = < < Erc20Minable as :: ink_lang :: ContractEnv > :: Env as :: ink_env :: Environment > :: MAX_EVENT_TOPICS ;
        fn __ink_ensure_max_event_topics<T>(_: T)
        where
            T: __ink_RenameBool,
            <T as __ink_RenameBool>::Type: CompliesWithTopicLimit,
        {
        }
        let _ = __ink_ensure_max_event_topics::<[(); (2usize <= __ink_MAX_EVENT_TOPICS) as usize]>;
    };
    #[cfg(not(feature = "ink-as-dependency"))]
    const _: () = {
        #[allow(non_camel_case_types)]
        pub enum __ink_CheckSatisfied {}
        pub enum EventTopicsWithinBounds {}
        impl ::ink_lang::True for __ink_CheckSatisfied {}
        #[doc(hidden)]
        pub trait CompliesWithTopicLimit {}
        impl CompliesWithTopicLimit for __ink_CheckSatisfied {}
        #[allow(non_camel_case_types)]
        pub trait __ink_RenameBool {
            type Type;
        }
        impl __ink_RenameBool for [(); true as usize] {
            type Type = __ink_CheckSatisfied;
        }
        impl __ink_RenameBool for [(); false as usize] {
            type Type = Approval;
        }
        #[allow(non_upper_case_globals)]
        const __ink_MAX_EVENT_TOPICS : usize = < < Erc20Minable as :: ink_lang :: ContractEnv > :: Env as :: ink_env :: Environment > :: MAX_EVENT_TOPICS ;
        fn __ink_ensure_max_event_topics<T>(_: T)
        where
            T: __ink_RenameBool,
            <T as __ink_RenameBool>::Type: CompliesWithTopicLimit,
        {
        }
        let _ = __ink_ensure_max_event_topics::<[(); (2usize <= __ink_MAX_EVENT_TOPICS) as usize]>;
    };
    #[cfg(not(feature = "ink-as-dependency"))]
    /// Event emitted when a token transfer occurs.
    pub struct Transfer {
        from: Option<AccountId>,
        to: Option<AccountId>,
        value: Balance,
    }
    const _: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate scale as _parity_scale_codec;
        impl _parity_scale_codec::Encode for Transfer {
            fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output + ?Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                _parity_scale_codec::Encode::encode_to(&self.from, __codec_dest_edqy);
                _parity_scale_codec::Encode::encode_to(&self.to, __codec_dest_edqy);
                _parity_scale_codec::Encode::encode_to(&self.value, __codec_dest_edqy);
            }
        }
        impl _parity_scale_codec::EncodeLike for Transfer {}
    };
    const _: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate scale as _parity_scale_codec;
        impl _parity_scale_codec::Decode for Transfer {
            fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, _parity_scale_codec::Error> {
                ::core::result::Result::Ok(Transfer {
                    from: {
                        let __codec_res_edqy =
                            <Option<AccountId> as _parity_scale_codec::Decode>::decode(
                                __codec_input_edqy,
                            );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Transfer::from`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    },
                    to: {
                        let __codec_res_edqy =
                            <Option<AccountId> as _parity_scale_codec::Decode>::decode(
                                __codec_input_edqy,
                            );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Transfer::to`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    },
                    value: {
                        let __codec_res_edqy =
                            <Balance as _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Transfer::value`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    },
                })
            }
        }
    };
    #[cfg(not(feature = "ink-as-dependency"))]
    /// Event emitted when an approval occurs that `spender` is allowed to withdraw
    /// up to the amount of `value` tokens from `owner`.
    pub struct Approval {
        owner: AccountId,
        spender: AccountId,
        value: Balance,
    }
    const _: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate scale as _parity_scale_codec;
        impl _parity_scale_codec::Encode for Approval {
            fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output + ?Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                _parity_scale_codec::Encode::encode_to(&self.owner, __codec_dest_edqy);
                _parity_scale_codec::Encode::encode_to(&self.spender, __codec_dest_edqy);
                _parity_scale_codec::Encode::encode_to(&self.value, __codec_dest_edqy);
            }
        }
        impl _parity_scale_codec::EncodeLike for Approval {}
    };
    const _: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate scale as _parity_scale_codec;
        impl _parity_scale_codec::Decode for Approval {
            fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, _parity_scale_codec::Error> {
                ::core::result::Result::Ok(Approval {
                    owner: {
                        let __codec_res_edqy =
                            <AccountId as _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Approval::owner`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    },
                    spender: {
                        let __codec_res_edqy =
                            <AccountId as _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Approval::spender`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    },
                    value: {
                        let __codec_res_edqy =
                            <Balance as _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Approval::value`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    },
                })
            }
        }
    };
    #[cfg(not(feature = "ink-as-dependency"))]
    const _: () = {
        impl ::ink_env::Topics for Transfer {
            type RemainingTopics = [::ink_env::topics::state::HasRemainingTopics; 3usize];
            fn topics<E, B>(
                &self,
                builder: ::ink_env::topics::TopicsBuilder<::ink_env::topics::state::Uninit, E, B>,
            ) -> <B as ::ink_env::topics::TopicsBuilderBackend<E>>::Output
            where
                E: ::ink_env::Environment,
                B: ::ink_env::topics::TopicsBuilderBackend<E>,
            {
                builder
                    .build::<Self>()
                    .push_topic::<::ink_env::topics::PrefixedValue<[u8; 22usize]>>(
                        &::ink_env::topics::PrefixedValue {
                            value: b"Erc20Minable::Transfer",
                            prefix: b"",
                        },
                    )
                    .push_topic::<::ink_env::topics::PrefixedValue<Option<AccountId>>>(
                        &::ink_env::topics::PrefixedValue {
                            value: &self.from,
                            prefix: b"Erc20Minable::Transfer::from",
                        },
                    )
                    .push_topic::<::ink_env::topics::PrefixedValue<Option<AccountId>>>(
                        &::ink_env::topics::PrefixedValue {
                            value: &self.to,
                            prefix: b"Erc20Minable::Transfer::to",
                        },
                    )
                    .finish()
            }
        }
    };
    #[cfg(not(feature = "ink-as-dependency"))]
    const _: () = {
        impl ::ink_env::Topics for Approval {
            type RemainingTopics = [::ink_env::topics::state::HasRemainingTopics; 3usize];
            fn topics<E, B>(
                &self,
                builder: ::ink_env::topics::TopicsBuilder<::ink_env::topics::state::Uninit, E, B>,
            ) -> <B as ::ink_env::topics::TopicsBuilderBackend<E>>::Output
            where
                E: ::ink_env::Environment,
                B: ::ink_env::topics::TopicsBuilderBackend<E>,
            {
                builder
                    .build::<Self>()
                    .push_topic::<::ink_env::topics::PrefixedValue<[u8; 22usize]>>(
                        &::ink_env::topics::PrefixedValue {
                            value: b"Erc20Minable::Approval",
                            prefix: b"",
                        },
                    )
                    .push_topic::<::ink_env::topics::PrefixedValue<AccountId>>(
                        &::ink_env::topics::PrefixedValue {
                            value: &self.owner,
                            prefix: b"Erc20Minable::Approval::owner",
                        },
                    )
                    .push_topic::<::ink_env::topics::PrefixedValue<AccountId>>(
                        &::ink_env::topics::PrefixedValue {
                            value: &self.spender,
                            prefix: b"Erc20Minable::Approval::spender",
                        },
                    )
                    .finish()
            }
        }
    };
    #[cfg(not(test))]
    #[cfg(not(feature = "ink-as-dependency"))]
    const _: () = {
        #[cfg(not(test))]
        #[no_mangle]
        fn deploy() -> u32 {
            ::ink_lang::DispatchRetCode::from(
                <Erc20Minable as ::ink_lang::DispatchUsingMode>::dispatch_using_mode(
                    ::ink_lang::DispatchMode::Instantiate,
                ),
            )
            .to_u32()
        }
        #[cfg(not(test))]
        #[no_mangle]
        fn call() -> u32 {
            if true {
                ::ink_lang::deny_payment::<<Erc20Minable as ::ink_lang::ContractEnv>::Env>()
                    .expect("caller transferred value even though all ink! message deny payments")
            }
            ::ink_lang::DispatchRetCode::from(
                <Erc20Minable as ::ink_lang::DispatchUsingMode>::dispatch_using_mode(
                    ::ink_lang::DispatchMode::Call,
                ),
            )
            .to_u32()
        }
        impl ::ink_lang::DispatchUsingMode for Erc20Minable {
            #[allow(unused_parens)]
            fn dispatch_using_mode(
                mode: ::ink_lang::DispatchMode,
            ) -> core::result::Result<(), ::ink_lang::DispatchError> {
                match mode { :: ink_lang :: DispatchMode :: Instantiate => { < < Erc20Minable as :: ink_lang :: ConstructorDispatcher > :: Type as :: ink_lang :: Execute > :: execute (:: ink_env :: decode_input :: < < Erc20Minable as :: ink_lang :: ConstructorDispatcher > :: Type > () . map_err (| _ | :: ink_lang :: DispatchError :: CouldNotReadInput) ?) } :: ink_lang :: DispatchMode :: Call => { < < Erc20Minable as :: ink_lang :: MessageDispatcher > :: Type as :: ink_lang :: Execute > :: execute (:: ink_env :: decode_input :: < < Erc20Minable as :: ink_lang :: MessageDispatcher > :: Type > () . map_err (| _ | :: ink_lang :: DispatchError :: CouldNotReadInput) ?) } }
            }
        }
        #[doc(hidden)]
        pub struct __ink_Msg<S> {
            marker: core::marker::PhantomData<fn() -> S>,
        }
        #[doc(hidden)]
        pub struct __ink_Constr<S> {
            marker: core::marker::PhantomData<fn() -> S>,
        }
        impl ::ink_lang::FnInput for __ink_Msg<[(); 234347066usize]> {
            type Input = ();
        }
        impl ::ink_lang::FnSelector for __ink_Msg<[(); 234347066usize]> {
            const SELECTOR: ::ink_env::call::Selector =
                ::ink_env::call::Selector::new([58u8, 218u8, 247u8, 13u8]);
        }
        impl ::ink_lang::FnState for __ink_Msg<[(); 234347066usize]> {
            type State = Erc20Minable;
        }
        impl ::ink_lang::FnOutput for __ink_Msg<[(); 234347066usize]> {
            #[allow(unused_parens)]
            type Output = String;
        }
        impl ::ink_lang::MessageRef for __ink_Msg<[(); 234347066usize]> {
            const CALLABLE: fn(
                &<Self as ::ink_lang::FnState>::State,
                <Self as ::ink_lang::FnInput>::Input,
            ) -> <Self as ::ink_lang::FnOutput>::Output = |state, _| <Erc20Minable>::name(state);
        }
        impl ::ink_lang::FnInput for __ink_Msg<[(); 1049874843usize]> {
            type Input = ();
        }
        impl ::ink_lang::FnSelector for __ink_Msg<[(); 1049874843usize]> {
            const SELECTOR: ::ink_env::call::Selector =
                ::ink_env::call::Selector::new([155u8, 209u8, 147u8, 62u8]);
        }
        impl ::ink_lang::FnState for __ink_Msg<[(); 1049874843usize]> {
            type State = Erc20Minable;
        }
        impl ::ink_lang::FnOutput for __ink_Msg<[(); 1049874843usize]> {
            #[allow(unused_parens)]
            type Output = String;
        }
        impl ::ink_lang::MessageRef for __ink_Msg<[(); 1049874843usize]> {
            const CALLABLE: fn(
                &<Self as ::ink_lang::FnState>::State,
                <Self as ::ink_lang::FnInput>::Input,
            ) -> <Self as ::ink_lang::FnOutput>::Output = |state, _| <Erc20Minable>::symbol(state);
        }
        impl ::ink_lang::FnInput for __ink_Msg<[(); 2275262593usize]> {
            type Input = ();
        }
        impl ::ink_lang::FnSelector for __ink_Msg<[(); 2275262593usize]> {
            const SELECTOR: ::ink_env::call::Selector =
                ::ink_env::call::Selector::new([129u8, 192u8, 157u8, 135u8]);
        }
        impl ::ink_lang::FnState for __ink_Msg<[(); 2275262593usize]> {
            type State = Erc20Minable;
        }
        impl ::ink_lang::FnOutput for __ink_Msg<[(); 2275262593usize]> {
            #[allow(unused_parens)]
            type Output = u8;
        }
        impl ::ink_lang::MessageRef for __ink_Msg<[(); 2275262593usize]> {
            const CALLABLE: fn(
                &<Self as ::ink_lang::FnState>::State,
                <Self as ::ink_lang::FnInput>::Input,
            ) -> <Self as ::ink_lang::FnOutput>::Output =
                |state, _| <Erc20Minable>::decimals(state);
        }
        impl ::ink_lang::FnInput for __ink_Msg<[(); 2826265563usize]> {
            type Input = ();
        }
        impl ::ink_lang::FnSelector for __ink_Msg<[(); 2826265563usize]> {
            const SELECTOR: ::ink_env::call::Selector =
                ::ink_env::call::Selector::new([219u8, 99u8, 117u8, 168u8]);
        }
        impl ::ink_lang::FnState for __ink_Msg<[(); 2826265563usize]> {
            type State = Erc20Minable;
        }
        impl ::ink_lang::FnOutput for __ink_Msg<[(); 2826265563usize]> {
            #[allow(unused_parens)]
            type Output = Balance;
        }
        impl ::ink_lang::MessageRef for __ink_Msg<[(); 2826265563usize]> {
            const CALLABLE: fn(
                &<Self as ::ink_lang::FnState>::State,
                <Self as ::ink_lang::FnInput>::Input,
            ) -> <Self as ::ink_lang::FnOutput>::Output =
                |state, _| <Erc20Minable>::total_supply(state);
        }
        impl ::ink_lang::FnInput for __ink_Msg<[(); 1448768783usize]> {
            type Input = AccountId;
        }
        impl ::ink_lang::FnSelector for __ink_Msg<[(); 1448768783usize]> {
            const SELECTOR: ::ink_env::call::Selector =
                ::ink_env::call::Selector::new([15u8, 117u8, 90u8, 86u8]);
        }
        impl ::ink_lang::FnState for __ink_Msg<[(); 1448768783usize]> {
            type State = Erc20Minable;
        }
        impl ::ink_lang::FnOutput for __ink_Msg<[(); 1448768783usize]> {
            #[allow(unused_parens)]
            type Output = Balance;
        }
        impl ::ink_lang::MessageRef for __ink_Msg<[(); 1448768783usize]> {
            const CALLABLE: fn(
                &<Self as ::ink_lang::FnState>::State,
                <Self as ::ink_lang::FnInput>::Input,
            ) -> <Self as ::ink_lang::FnOutput>::Output =
                |state, __ink_binding_0| <Erc20Minable>::balance_of(state, __ink_binding_0);
        }
        impl ::ink_lang::FnInput for __ink_Msg<[(); 1578500202usize]> {
            type Input = (AccountId, AccountId);
        }
        impl ::ink_lang::FnSelector for __ink_Msg<[(); 1578500202usize]> {
            const SELECTOR: ::ink_env::call::Selector =
                ::ink_env::call::Selector::new([106u8, 0u8, 22u8, 94u8]);
        }
        impl ::ink_lang::FnState for __ink_Msg<[(); 1578500202usize]> {
            type State = Erc20Minable;
        }
        impl ::ink_lang::FnOutput for __ink_Msg<[(); 1578500202usize]> {
            #[allow(unused_parens)]
            type Output = Balance;
        }
        impl ::ink_lang::MessageRef for __ink_Msg<[(); 1578500202usize]> {
            const CALLABLE: fn(
                &<Self as ::ink_lang::FnState>::State,
                <Self as ::ink_lang::FnInput>::Input,
            ) -> <Self as ::ink_lang::FnOutput>::Output =
                |state, (__ink_binding_0, __ink_binding_1)| {
                    <Erc20Minable>::allowance(state, __ink_binding_0, __ink_binding_1)
                };
        }
        impl ::ink_lang::FnInput for __ink_Msg<[(); 2707267972usize]> {
            type Input = (AccountId, Balance);
        }
        impl ::ink_lang::FnSelector for __ink_Msg<[(); 2707267972usize]> {
            const SELECTOR: ::ink_env::call::Selector =
                ::ink_env::call::Selector::new([132u8, 161u8, 93u8, 161u8]);
        }
        impl ::ink_lang::FnState for __ink_Msg<[(); 2707267972usize]> {
            type State = Erc20Minable;
        }
        impl ::ink_lang::FnOutput for __ink_Msg<[(); 2707267972usize]> {
            #[allow(unused_parens)]
            type Output = Result<()>;
        }
        impl ::ink_lang::MessageMut for __ink_Msg<[(); 2707267972usize]> {
            const CALLABLE: fn(
                &mut <Self as ::ink_lang::FnState>::State,
                <Self as ::ink_lang::FnInput>::Input,
            ) -> <Self as ::ink_lang::FnOutput>::Output =
                |state, (__ink_binding_0, __ink_binding_1)| {
                    <Erc20Minable>::transfer(state, __ink_binding_0, __ink_binding_1)
                };
        }
        impl ::ink_lang::FnInput for __ink_Msg<[(); 2691043944usize]> {
            type Input = (AccountId, Balance);
        }
        impl ::ink_lang::FnSelector for __ink_Msg<[(); 2691043944usize]> {
            const SELECTOR: ::ink_env::call::Selector =
                ::ink_env::call::Selector::new([104u8, 18u8, 102u8, 160u8]);
        }
        impl ::ink_lang::FnState for __ink_Msg<[(); 2691043944usize]> {
            type State = Erc20Minable;
        }
        impl ::ink_lang::FnOutput for __ink_Msg<[(); 2691043944usize]> {
            #[allow(unused_parens)]
            type Output = Result<()>;
        }
        impl ::ink_lang::MessageMut for __ink_Msg<[(); 2691043944usize]> {
            const CALLABLE: fn(
                &mut <Self as ::ink_lang::FnState>::State,
                <Self as ::ink_lang::FnInput>::Input,
            ) -> <Self as ::ink_lang::FnOutput>::Output =
                |state, (__ink_binding_0, __ink_binding_1)| {
                    <Erc20Minable>::approve(state, __ink_binding_0, __ink_binding_1)
                };
        }
        impl ::ink_lang::FnInput for __ink_Msg<[(); 409942283usize]> {
            type Input = (AccountId, AccountId, Balance);
        }
        impl ::ink_lang::FnSelector for __ink_Msg<[(); 409942283usize]> {
            const SELECTOR: ::ink_env::call::Selector =
                ::ink_env::call::Selector::new([11u8, 57u8, 111u8, 24u8]);
        }
        impl ::ink_lang::FnState for __ink_Msg<[(); 409942283usize]> {
            type State = Erc20Minable;
        }
        impl ::ink_lang::FnOutput for __ink_Msg<[(); 409942283usize]> {
            #[allow(unused_parens)]
            type Output = Result<()>;
        }
        impl ::ink_lang::MessageMut for __ink_Msg<[(); 409942283usize]> {
            const CALLABLE: fn(
                &mut <Self as ::ink_lang::FnState>::State,
                <Self as ::ink_lang::FnInput>::Input,
            ) -> <Self as ::ink_lang::FnOutput>::Output =
                |state, (__ink_binding_0, __ink_binding_1, __ink_binding_2)| {
                    <Erc20Minable>::transfer_from(
                        state,
                        __ink_binding_0,
                        __ink_binding_1,
                        __ink_binding_2,
                    )
                };
        }
        impl ::ink_lang::FnInput for __ink_Msg<[(); 3292512530usize]> {
            type Input = (AccountId, Balance);
        }
        impl ::ink_lang::FnSelector for __ink_Msg<[(); 3292512530usize]> {
            const SELECTOR: ::ink_env::call::Selector =
                ::ink_env::call::Selector::new([18u8, 193u8, 63u8, 196u8]);
        }
        impl ::ink_lang::FnState for __ink_Msg<[(); 3292512530usize]> {
            type State = Erc20Minable;
        }
        impl ::ink_lang::FnOutput for __ink_Msg<[(); 3292512530usize]> {
            #[allow(unused_parens)]
            type Output = ();
        }
        impl ::ink_lang::MessageMut for __ink_Msg<[(); 3292512530usize]> {
            const CALLABLE: fn(
                &mut <Self as ::ink_lang::FnState>::State,
                <Self as ::ink_lang::FnInput>::Input,
            ) -> <Self as ::ink_lang::FnOutput>::Output =
                |state, (__ink_binding_0, __ink_binding_1)| {
                    <Erc20Minable>::miner(state, __ink_binding_0, __ink_binding_1)
                };
        }
        impl ::ink_lang::FnInput for __ink_Constr<[(); 1587392155usize]> {
            type Input = Balance;
        }
        impl ::ink_lang::FnSelector for __ink_Constr<[(); 1587392155usize]> {
            const SELECTOR: ::ink_env::call::Selector =
                ::ink_env::call::Selector::new([155u8, 174u8, 157u8, 94u8]);
        }
        impl ::ink_lang::FnState for __ink_Constr<[(); 1587392155usize]> {
            type State = Erc20Minable;
        }
        impl ::ink_lang::Constructor for __ink_Constr<[(); 1587392155usize]> {
            const CALLABLE: fn(
                <Self as ::ink_lang::FnInput>::Input,
            ) -> <Self as ::ink_lang::FnState>::State =
                |__ink_binding_0| <Erc20Minable>::new(__ink_binding_0);
        }
        const _: () = {
            #[doc(hidden)]
            pub enum __ink_MessageDispatchEnum {
                __ink_Message_0x3adaf70d(),
                __ink_Message_0x9bd1933e(),
                __ink_Message_0x81c09d87(),
                __ink_Message_0xdb6375a8(),
                __ink_Message_0x0f755a56(AccountId),
                __ink_Message_0x6a00165e(AccountId, AccountId),
                __ink_Message_0x84a15da1(AccountId, Balance),
                __ink_Message_0x681266a0(AccountId, Balance),
                __ink_Message_0x0b396f18(AccountId, AccountId, Balance),
                __ink_Message_0x12c13fc4(AccountId, Balance),
            }
            impl ::ink_lang::MessageDispatcher for Erc20Minable {
                type Type = __ink_MessageDispatchEnum;
            }
            impl ::scale::Decode for __ink_MessageDispatchEnum {
                fn decode<I: ::scale::Input>(
                    input: &mut I,
                ) -> ::core::result::Result<Self, ::scale::Error> {
                    match <[u8; 4] as ::scale::Decode>::decode(input)? {
                        [58u8, 218u8, 247u8, 13u8] => Ok(Self::__ink_Message_0x3adaf70d()),
                        [155u8, 209u8, 147u8, 62u8] => Ok(Self::__ink_Message_0x9bd1933e()),
                        [129u8, 192u8, 157u8, 135u8] => Ok(Self::__ink_Message_0x81c09d87()),
                        [219u8, 99u8, 117u8, 168u8] => Ok(Self::__ink_Message_0xdb6375a8()),
                        [15u8, 117u8, 90u8, 86u8] => Ok(Self::__ink_Message_0x0f755a56(
                            <AccountId as ::scale::Decode>::decode(input)?,
                        )),
                        [106u8, 0u8, 22u8, 94u8] => Ok(Self::__ink_Message_0x6a00165e(
                            <AccountId as ::scale::Decode>::decode(input)?,
                            <AccountId as ::scale::Decode>::decode(input)?,
                        )),
                        [132u8, 161u8, 93u8, 161u8] => Ok(Self::__ink_Message_0x84a15da1(
                            <AccountId as ::scale::Decode>::decode(input)?,
                            <Balance as ::scale::Decode>::decode(input)?,
                        )),
                        [104u8, 18u8, 102u8, 160u8] => Ok(Self::__ink_Message_0x681266a0(
                            <AccountId as ::scale::Decode>::decode(input)?,
                            <Balance as ::scale::Decode>::decode(input)?,
                        )),
                        [11u8, 57u8, 111u8, 24u8] => Ok(Self::__ink_Message_0x0b396f18(
                            <AccountId as ::scale::Decode>::decode(input)?,
                            <AccountId as ::scale::Decode>::decode(input)?,
                            <Balance as ::scale::Decode>::decode(input)?,
                        )),
                        [18u8, 193u8, 63u8, 196u8] => Ok(Self::__ink_Message_0x12c13fc4(
                            <AccountId as ::scale::Decode>::decode(input)?,
                            <Balance as ::scale::Decode>::decode(input)?,
                        )),
                        _invalid => Err(::scale::Error::from(
                            "encountered unknown ink! message selector",
                        )),
                    }
                }
            }
            impl ::ink_lang::Execute for __ink_MessageDispatchEnum {
                fn execute(self) -> ::core::result::Result<(), ::ink_lang::DispatchError> {
                    match self {
                        Self::__ink_Message_0x3adaf70d() => ::ink_lang::execute_message::<
                            <Erc20Minable as ::ink_lang::ContractEnv>::Env,
                            __ink_Msg<[(); 234347066usize]>,
                            _,
                        >(
                            ::ink_lang::AcceptsPayments(true),
                            ::ink_lang::EnablesDynamicStorageAllocator(false),
                            move |state: &Erc20Minable| {
                                < __ink_Msg < [() ; 234347066usize] > as :: ink_lang :: MessageRef > :: CALLABLE (state , ())
                            },
                        ),
                        Self::__ink_Message_0x9bd1933e() => ::ink_lang::execute_message::<
                            <Erc20Minable as ::ink_lang::ContractEnv>::Env,
                            __ink_Msg<[(); 1049874843usize]>,
                            _,
                        >(
                            ::ink_lang::AcceptsPayments(true),
                            ::ink_lang::EnablesDynamicStorageAllocator(false),
                            move |state: &Erc20Minable| {
                                < __ink_Msg < [() ; 1049874843usize] > as :: ink_lang :: MessageRef > :: CALLABLE (state , ())
                            },
                        ),
                        Self::__ink_Message_0x81c09d87() => ::ink_lang::execute_message::<
                            <Erc20Minable as ::ink_lang::ContractEnv>::Env,
                            __ink_Msg<[(); 2275262593usize]>,
                            _,
                        >(
                            ::ink_lang::AcceptsPayments(true),
                            ::ink_lang::EnablesDynamicStorageAllocator(false),
                            move |state: &Erc20Minable| {
                                < __ink_Msg < [() ; 2275262593usize] > as :: ink_lang :: MessageRef > :: CALLABLE (state , ())
                            },
                        ),
                        Self::__ink_Message_0xdb6375a8() => ::ink_lang::execute_message::<
                            <Erc20Minable as ::ink_lang::ContractEnv>::Env,
                            __ink_Msg<[(); 2826265563usize]>,
                            _,
                        >(
                            ::ink_lang::AcceptsPayments(true),
                            ::ink_lang::EnablesDynamicStorageAllocator(false),
                            move |state: &Erc20Minable| {
                                < __ink_Msg < [() ; 2826265563usize] > as :: ink_lang :: MessageRef > :: CALLABLE (state , ())
                            },
                        ),
                        Self::__ink_Message_0x0f755a56(owner) => ::ink_lang::execute_message::<
                            <Erc20Minable as ::ink_lang::ContractEnv>::Env,
                            __ink_Msg<[(); 1448768783usize]>,
                            _,
                        >(
                            ::ink_lang::AcceptsPayments(true),
                            ::ink_lang::EnablesDynamicStorageAllocator(false),
                            move |state: &Erc20Minable| {
                                < __ink_Msg < [() ; 1448768783usize] > as :: ink_lang :: MessageRef > :: CALLABLE (state , owner)
                            },
                        ),
                        Self::__ink_Message_0x6a00165e(owner, spender) => {
                            ::ink_lang::execute_message::<
                                <Erc20Minable as ::ink_lang::ContractEnv>::Env,
                                __ink_Msg<[(); 1578500202usize]>,
                                _,
                            >(
                                ::ink_lang::AcceptsPayments(true),
                                ::ink_lang::EnablesDynamicStorageAllocator(false),
                                move |state: &Erc20Minable| {
                                    < __ink_Msg < [() ; 1578500202usize] > as :: ink_lang :: MessageRef > :: CALLABLE (state , (owner , spender))
                                },
                            )
                        }
                        Self::__ink_Message_0x84a15da1(to, value) => {
                            ::ink_lang::execute_message_mut::<
                                <Erc20Minable as ::ink_lang::ContractEnv>::Env,
                                __ink_Msg<[(); 2707267972usize]>,
                                _,
                            >(
                                ::ink_lang::AcceptsPayments(true),
                                ::ink_lang::EnablesDynamicStorageAllocator(false),
                                move |state: &mut Erc20Minable| {
                                    < __ink_Msg < [() ; 2707267972usize] > as :: ink_lang :: MessageMut > :: CALLABLE (state , (to , value))
                                },
                            )
                        }
                        Self::__ink_Message_0x681266a0(spender, value) => {
                            ::ink_lang::execute_message_mut::<
                                <Erc20Minable as ::ink_lang::ContractEnv>::Env,
                                __ink_Msg<[(); 2691043944usize]>,
                                _,
                            >(
                                ::ink_lang::AcceptsPayments(true),
                                ::ink_lang::EnablesDynamicStorageAllocator(false),
                                move |state: &mut Erc20Minable| {
                                    < __ink_Msg < [() ; 2691043944usize] > as :: ink_lang :: MessageMut > :: CALLABLE (state , (spender , value))
                                },
                            )
                        }
                        Self::__ink_Message_0x0b396f18(from, to, value) => {
                            ::ink_lang::execute_message_mut::<
                                <Erc20Minable as ::ink_lang::ContractEnv>::Env,
                                __ink_Msg<[(); 409942283usize]>,
                                _,
                            >(
                                ::ink_lang::AcceptsPayments(true),
                                ::ink_lang::EnablesDynamicStorageAllocator(false),
                                move |state: &mut Erc20Minable| {
                                    < __ink_Msg < [() ; 409942283usize] > as :: ink_lang :: MessageMut > :: CALLABLE (state , (from , to , value))
                                },
                            )
                        }
                        Self::__ink_Message_0x12c13fc4(to, value) => {
                            ::ink_lang::execute_message_mut::<
                                <Erc20Minable as ::ink_lang::ContractEnv>::Env,
                                __ink_Msg<[(); 3292512530usize]>,
                                _,
                            >(
                                ::ink_lang::AcceptsPayments(true),
                                ::ink_lang::EnablesDynamicStorageAllocator(false),
                                move |state: &mut Erc20Minable| {
                                    < __ink_Msg < [() ; 3292512530usize] > as :: ink_lang :: MessageMut > :: CALLABLE (state , (to , value))
                                },
                            )
                        }
                    }
                }
            }
        };
        const _: () = {
            #[doc(hidden)]
            pub enum __ink_ConstructorDispatchEnum {
                __ink_Constructor_0x9bae9d5e(Balance),
            }
            impl ::ink_lang::ConstructorDispatcher for Erc20Minable {
                type Type = __ink_ConstructorDispatchEnum;
            }
            impl ::scale::Decode for __ink_ConstructorDispatchEnum {
                fn decode<I: ::scale::Input>(
                    input: &mut I,
                ) -> ::core::result::Result<Self, ::scale::Error> {
                    match <[u8; 4] as ::scale::Decode>::decode(input)? {
                        [155u8, 174u8, 157u8, 94u8] => Ok(Self::__ink_Constructor_0x9bae9d5e(
                            <Balance as ::scale::Decode>::decode(input)?,
                        )),
                        _invalid => Err(::scale::Error::from(
                            "encountered unknown ink! constructor selector",
                        )),
                    }
                }
            }
            impl ::ink_lang::Execute for __ink_ConstructorDispatchEnum {
                fn execute(self) -> ::core::result::Result<(), ::ink_lang::DispatchError> {
                    match self {
                        Self::__ink_Constructor_0x9bae9d5e(initial_supply) => {
                            ::ink_lang::execute_constructor::<__ink_Constr<[(); 1587392155usize]>, _>(
                                ::ink_lang::EnablesDynamicStorageAllocator(false),
                                move || {
                                    < __ink_Constr < [() ; 1587392155usize] > as :: ink_lang :: Constructor > :: CALLABLE (initial_supply)
                                },
                            )
                        }
                    }
                }
            }
        };
    };
    #[cfg(not(feature = "ink-as-dependency"))]
    const _: () = { use :: ink_lang :: { Env , EmitEvent , StaticEnv } ; const _ : fn () = | | { { trait TypeEq { type This : ? Sized ; } impl < T : ? Sized > TypeEq for T { type This = Self ; } fn assert_type_eq_all < T , U > () where T : ? Sized + TypeEq < This = U > , U : ? Sized { } assert_type_eq_all :: < Erc20Minable , Erc20Minable > () ; } } ; impl Erc20Minable { # [doc = " Creates a new ERC-20 contract with the specified initial supply."] pub fn new (initial_supply : Balance) -> Self { let mut instance = Self { erc20 : erc20 :: Data :: new () , } ; erc20 :: Impl :: init (& mut instance , String :: from ("MetisTestToken") , String :: from ("MET") , initial_supply) ; instance } # [doc = " Returns the name of the token."] pub fn name (& self) -> String { erc20 :: Impl :: name (self) } # [doc = " Returns the symbol of the token, usually a shorter version of the name."] pub fn symbol (& self) -> String { erc20 :: Impl :: symbol (self) } pub fn decimals (& self) -> u8 { 18_u8 } # [doc = " Returns the total token supply."] pub fn total_supply (& self) -> Balance { erc20 :: Impl :: total_supply (self) } # [doc = " Returns the account balance for the specified `owner`."] # [doc = ""] # [doc = " Returns `0` if the account is non-existent."] pub fn balance_of (& self , owner : AccountId) -> Balance { erc20 :: Impl :: balance_of (self , & owner) } # [doc = " Returns the amount which `spender` is still allowed to withdraw from `owner`."] # [doc = ""] # [doc = " Returns `0` if no allowance has been set `0`."] pub fn allowance (& self , owner : AccountId , spender : AccountId) -> Balance { erc20 :: Impl :: allowance (self , & owner , & spender) } # [doc = " Transfers `value` amount of tokens from the caller\'s account to account `to`."] # [doc = ""] # [doc = " On success a `Transfer` event is emitted."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Returns `InsufficientBalance` error if there are not enough tokens on"] # [doc = " the caller\'s account balance."] pub fn transfer (& mut self , to : AccountId , value : Balance) -> Result < () > { erc20 :: Impl :: transfer (self , & to , value) } # [doc = " Allows `spender` to withdraw from the caller\'s account multiple times, up to"] # [doc = " the `value` amount."] # [doc = ""] # [doc = " If this function is called again it overwrites the current allowance with `value`."] # [doc = ""] # [doc = " An `Approval` event is emitted."] pub fn approve (& mut self , spender : AccountId , value : Balance) -> Result < () > { erc20 :: Impl :: approve (self , & spender , value) } # [doc = " Transfers `value` tokens on the behalf of `from` to the account `to`."] # [doc = ""] # [doc = " This can be used to allow a contract to transfer tokens on ones behalf and/or"] # [doc = " to charge fees in sub-currencies, for example."] # [doc = ""] # [doc = " On success a `Transfer` event is emitted."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Returns `InsufficientAllowance` error if there are not enough tokens allowed"] # [doc = " for the caller to withdraw from `from`."] # [doc = ""] # [doc = " Returns `InsufficientBalance` error if there are not enough tokens on"] # [doc = " the the account balance of `from`."] pub fn transfer_from (& mut self , from : AccountId , to : AccountId , value : Balance) -> Result < () > { erc20 :: Impl :: transfer_from (self , & from , & to , value) } pub fn miner (& mut self , to : AccountId , value : Balance) { let enty = self . erc20 . balances . entry (to) ; let old_balance = enty . or_insert (0) ; * old_balance += value ; * self . erc20 . total_supply += value ; } } };
    #[cfg(feature = "std")]
    #[cfg(not(feature = "ink-as-dependency"))]
    const _: () = {
        #[no_mangle]
        pub fn __ink_generate_metadata() -> ::ink_metadata::InkProject {
            let contract: ::ink_metadata::ContractSpec = {
                :: ink_metadata :: ContractSpec :: new () . constructors (< [_] > :: into_vec (box [:: ink_metadata :: ConstructorSpec :: from_name ("new") . selector ([155u8 , 174u8 , 157u8 , 94u8]) . args (< [_] > :: into_vec (box [:: ink_metadata :: MessageParamSpec :: new ("initial_supply") . of_type (:: ink_metadata :: TypeSpec :: with_name_segs :: < Balance , _ > (< [_] > :: into_vec (box ["Balance"]) . into_iter () . map (AsRef :: as_ref))) . done ()])) . docs (< [_] > :: into_vec (box [" Creates a new ERC-20 contract with the specified initial supply."])) . done ()])) . messages (< [_] > :: into_vec (box [:: ink_metadata :: MessageSpec :: from_name ("name") . selector ([58u8 , 218u8 , 247u8 , 13u8]) . args (:: alloc :: vec :: Vec :: new ()) . returns (:: ink_metadata :: ReturnTypeSpec :: new (:: ink_metadata :: TypeSpec :: with_name_segs :: < String , _ > (< [_] > :: into_vec (box ["String"]) . into_iter () . map (AsRef :: as_ref)))) . mutates (false) . payable (false) . docs (< [_] > :: into_vec (box [" Returns the name of the token."])) . done () , :: ink_metadata :: MessageSpec :: from_name ("symbol") . selector ([155u8 , 209u8 , 147u8 , 62u8]) . args (:: alloc :: vec :: Vec :: new ()) . returns (:: ink_metadata :: ReturnTypeSpec :: new (:: ink_metadata :: TypeSpec :: with_name_segs :: < String , _ > (< [_] > :: into_vec (box ["String"]) . into_iter () . map (AsRef :: as_ref)))) . mutates (false) . payable (false) . docs (< [_] > :: into_vec (box [" Returns the symbol of the token, usually a shorter version of the name."])) . done () , :: ink_metadata :: MessageSpec :: from_name ("decimals") . selector ([129u8 , 192u8 , 157u8 , 135u8]) . args (:: alloc :: vec :: Vec :: new ()) . returns (:: ink_metadata :: ReturnTypeSpec :: new (:: ink_metadata :: TypeSpec :: with_name_segs :: < u8 , _ > (< [_] > :: into_vec (box ["u8"]) . into_iter () . map (AsRef :: as_ref)))) . mutates (false) . payable (false) . docs (:: alloc :: vec :: Vec :: new ()) . done () , :: ink_metadata :: MessageSpec :: from_name ("total_supply") . selector ([219u8 , 99u8 , 117u8 , 168u8]) . args (:: alloc :: vec :: Vec :: new ()) . returns (:: ink_metadata :: ReturnTypeSpec :: new (:: ink_metadata :: TypeSpec :: with_name_segs :: < Balance , _ > (< [_] > :: into_vec (box ["Balance"]) . into_iter () . map (AsRef :: as_ref)))) . mutates (false) . payable (false) . docs (< [_] > :: into_vec (box [" Returns the total token supply."])) . done () , :: ink_metadata :: MessageSpec :: from_name ("balance_of") . selector ([15u8 , 117u8 , 90u8 , 86u8]) . args (< [_] > :: into_vec (box [:: ink_metadata :: MessageParamSpec :: new ("owner") . of_type (:: ink_metadata :: TypeSpec :: with_name_segs :: < AccountId , _ > (< [_] > :: into_vec (box ["AccountId"]) . into_iter () . map (AsRef :: as_ref))) . done ()])) . returns (:: ink_metadata :: ReturnTypeSpec :: new (:: ink_metadata :: TypeSpec :: with_name_segs :: < Balance , _ > (< [_] > :: into_vec (box ["Balance"]) . into_iter () . map (AsRef :: as_ref)))) . mutates (false) . payable (false) . docs (< [_] > :: into_vec (box [" Returns the account balance for the specified `owner`." , "" , " Returns `0` if the account is non-existent."])) . done () , :: ink_metadata :: MessageSpec :: from_name ("allowance") . selector ([106u8 , 0u8 , 22u8 , 94u8]) . args (< [_] > :: into_vec (box [:: ink_metadata :: MessageParamSpec :: new ("owner") . of_type (:: ink_metadata :: TypeSpec :: with_name_segs :: < AccountId , _ > (< [_] > :: into_vec (box ["AccountId"]) . into_iter () . map (AsRef :: as_ref))) . done () , :: ink_metadata :: MessageParamSpec :: new ("spender") . of_type (:: ink_metadata :: TypeSpec :: with_name_segs :: < AccountId , _ > (< [_] > :: into_vec (box ["AccountId"]) . into_iter () . map (AsRef :: as_ref))) . done ()])) . returns (:: ink_metadata :: ReturnTypeSpec :: new (:: ink_metadata :: TypeSpec :: with_name_segs :: < Balance , _ > (< [_] > :: into_vec (box ["Balance"]) . into_iter () . map (AsRef :: as_ref)))) . mutates (false) . payable (false) . docs (< [_] > :: into_vec (box [" Returns the amount which `spender` is still allowed to withdraw from `owner`." , "" , " Returns `0` if no allowance has been set `0`."])) . done () , :: ink_metadata :: MessageSpec :: from_name ("transfer") . selector ([132u8 , 161u8 , 93u8 , 161u8]) . args (< [_] > :: into_vec (box [:: ink_metadata :: MessageParamSpec :: new ("to") . of_type (:: ink_metadata :: TypeSpec :: with_name_segs :: < AccountId , _ > (< [_] > :: into_vec (box ["AccountId"]) . into_iter () . map (AsRef :: as_ref))) . done () , :: ink_metadata :: MessageParamSpec :: new ("value") . of_type (:: ink_metadata :: TypeSpec :: with_name_segs :: < Balance , _ > (< [_] > :: into_vec (box ["Balance"]) . into_iter () . map (AsRef :: as_ref))) . done ()])) . returns (:: ink_metadata :: ReturnTypeSpec :: new (:: ink_metadata :: TypeSpec :: with_name_segs :: < Result < () > , _ > (< [_] > :: into_vec (box ["Result"]) . into_iter () . map (AsRef :: as_ref)))) . mutates (true) . payable (false) . docs (< [_] > :: into_vec (box [" Transfers `value` amount of tokens from the caller\'s account to account `to`." , "" , " On success a `Transfer` event is emitted." , "" , " # Errors" , "" , " Returns `InsufficientBalance` error if there are not enough tokens on" , " the caller\'s account balance."])) . done () , :: ink_metadata :: MessageSpec :: from_name ("approve") . selector ([104u8 , 18u8 , 102u8 , 160u8]) . args (< [_] > :: into_vec (box [:: ink_metadata :: MessageParamSpec :: new ("spender") . of_type (:: ink_metadata :: TypeSpec :: with_name_segs :: < AccountId , _ > (< [_] > :: into_vec (box ["AccountId"]) . into_iter () . map (AsRef :: as_ref))) . done () , :: ink_metadata :: MessageParamSpec :: new ("value") . of_type (:: ink_metadata :: TypeSpec :: with_name_segs :: < Balance , _ > (< [_] > :: into_vec (box ["Balance"]) . into_iter () . map (AsRef :: as_ref))) . done ()])) . returns (:: ink_metadata :: ReturnTypeSpec :: new (:: ink_metadata :: TypeSpec :: with_name_segs :: < Result < () > , _ > (< [_] > :: into_vec (box ["Result"]) . into_iter () . map (AsRef :: as_ref)))) . mutates (true) . payable (false) . docs (< [_] > :: into_vec (box [" Allows `spender` to withdraw from the caller\'s account multiple times, up to" , " the `value` amount." , "" , " If this function is called again it overwrites the current allowance with `value`." , "" , " An `Approval` event is emitted."])) . done () , :: ink_metadata :: MessageSpec :: from_name ("transfer_from") . selector ([11u8 , 57u8 , 111u8 , 24u8]) . args (< [_] > :: into_vec (box [:: ink_metadata :: MessageParamSpec :: new ("from") . of_type (:: ink_metadata :: TypeSpec :: with_name_segs :: < AccountId , _ > (< [_] > :: into_vec (box ["AccountId"]) . into_iter () . map (AsRef :: as_ref))) . done () , :: ink_metadata :: MessageParamSpec :: new ("to") . of_type (:: ink_metadata :: TypeSpec :: with_name_segs :: < AccountId , _ > (< [_] > :: into_vec (box ["AccountId"]) . into_iter () . map (AsRef :: as_ref))) . done () , :: ink_metadata :: MessageParamSpec :: new ("value") . of_type (:: ink_metadata :: TypeSpec :: with_name_segs :: < Balance , _ > (< [_] > :: into_vec (box ["Balance"]) . into_iter () . map (AsRef :: as_ref))) . done ()])) . returns (:: ink_metadata :: ReturnTypeSpec :: new (:: ink_metadata :: TypeSpec :: with_name_segs :: < Result < () > , _ > (< [_] > :: into_vec (box ["Result"]) . into_iter () . map (AsRef :: as_ref)))) . mutates (true) . payable (false) . docs (< [_] > :: into_vec (box [" Transfers `value` tokens on the behalf of `from` to the account `to`." , "" , " This can be used to allow a contract to transfer tokens on ones behalf and/or" , " to charge fees in sub-currencies, for example." , "" , " On success a `Transfer` event is emitted." , "" , " # Errors" , "" , " Returns `InsufficientAllowance` error if there are not enough tokens allowed" , " for the caller to withdraw from `from`." , "" , " Returns `InsufficientBalance` error if there are not enough tokens on" , " the the account balance of `from`."])) . done () , :: ink_metadata :: MessageSpec :: from_name ("miner") . selector ([18u8 , 193u8 , 63u8 , 196u8]) . args (< [_] > :: into_vec (box [:: ink_metadata :: MessageParamSpec :: new ("to") . of_type (:: ink_metadata :: TypeSpec :: with_name_segs :: < AccountId , _ > (< [_] > :: into_vec (box ["AccountId"]) . into_iter () . map (AsRef :: as_ref))) . done () , :: ink_metadata :: MessageParamSpec :: new ("value") . of_type (:: ink_metadata :: TypeSpec :: with_name_segs :: < Balance , _ > (< [_] > :: into_vec (box ["Balance"]) . into_iter () . map (AsRef :: as_ref))) . done ()])) . returns (:: ink_metadata :: ReturnTypeSpec :: new (None)) . mutates (true) . payable (false) . docs (:: alloc :: vec :: Vec :: new ()) . done ()])) . events (< [_] > :: into_vec (box [:: ink_metadata :: EventSpec :: new ("Transfer") . args (< [_] > :: into_vec (box [:: ink_metadata :: EventParamSpec :: new ("from") . of_type (:: ink_metadata :: TypeSpec :: with_name_segs :: < Option < AccountId > , _ > (< [_] > :: into_vec (box ["Option"]) . into_iter () . map (AsRef :: as_ref))) . indexed (true) . docs (:: alloc :: vec :: Vec :: new ()) . done () , :: ink_metadata :: EventParamSpec :: new ("to") . of_type (:: ink_metadata :: TypeSpec :: with_name_segs :: < Option < AccountId > , _ > (< [_] > :: into_vec (box ["Option"]) . into_iter () . map (AsRef :: as_ref))) . indexed (true) . docs (:: alloc :: vec :: Vec :: new ()) . done () , :: ink_metadata :: EventParamSpec :: new ("value") . of_type (:: ink_metadata :: TypeSpec :: with_name_segs :: < Balance , _ > (< [_] > :: into_vec (box ["Balance"]) . into_iter () . map (AsRef :: as_ref))) . indexed (false) . docs (:: alloc :: vec :: Vec :: new ()) . done ()])) . docs (< [_] > :: into_vec (box [" Event emitted when a token transfer occurs."])) . done () , :: ink_metadata :: EventSpec :: new ("Approval") . args (< [_] > :: into_vec (box [:: ink_metadata :: EventParamSpec :: new ("owner") . of_type (:: ink_metadata :: TypeSpec :: with_name_segs :: < AccountId , _ > (< [_] > :: into_vec (box ["AccountId"]) . into_iter () . map (AsRef :: as_ref))) . indexed (true) . docs (:: alloc :: vec :: Vec :: new ()) . done () , :: ink_metadata :: EventParamSpec :: new ("spender") . of_type (:: ink_metadata :: TypeSpec :: with_name_segs :: < AccountId , _ > (< [_] > :: into_vec (box ["AccountId"]) . into_iter () . map (AsRef :: as_ref))) . indexed (true) . docs (:: alloc :: vec :: Vec :: new ()) . done () , :: ink_metadata :: EventParamSpec :: new ("value") . of_type (:: ink_metadata :: TypeSpec :: with_name_segs :: < Balance , _ > (< [_] > :: into_vec (box ["Balance"]) . into_iter () . map (AsRef :: as_ref))) . indexed (false) . docs (:: alloc :: vec :: Vec :: new ()) . done ()])) . docs (< [_] > :: into_vec (box [" Event emitted when an approval occurs that `spender` is allowed to withdraw" , " up to the amount of `value` tokens from `owner`."])) . done ()])) . docs (:: alloc :: vec :: Vec :: new ()) . done ()
            };
            let layout: ::ink_metadata::layout::Layout = {
                <Erc20Minable as ::ink_storage::traits::StorageLayout>::layout(
                    &mut ::ink_primitives::KeyPtr::from(::ink_primitives::Key::from([0x00; 32])),
                )
            };
            ::ink_metadata::InkProject::new(layout, contract)
        }
    };
    use super::String;
    use erc20::Result;
    use metis_erc20 as erc20;
    use metis_lang::{import, metis};
    impl erc20::Impl<Self> for Erc20Minable {
        fn _before_token_transfer(
            &mut self,
            _: &AccountId,
            _: &AccountId,
            _: Balance,
        ) -> Result<()> {
            Ok(())
        }
    }
    #[cfg(not(feature = "ink-as-dependency"))]
    use ::ink_lang::{EmitEvent, Env, StaticEnv};
    #[cfg(not(feature = "ink-as-dependency"))]
    impl metis_lang::Env for Erc20Minable {
        type AccountId = <::ink_env::DefaultEnvironment as ::ink_env::Environment>::AccountId;
        type Balance = <::ink_env::DefaultEnvironment as ::ink_env::Environment>::Balance;
        type Hash = <::ink_env::DefaultEnvironment as ::ink_env::Environment>::Hash;
        type Timestamp = <::ink_env::DefaultEnvironment as ::ink_env::Environment>::Timestamp;
        type BlockNumber = <::ink_env::DefaultEnvironment as ::ink_env::Environment>::BlockNumber;
    }
    #[cfg(not(feature = "ink-as-dependency"))]
    impl metis_lang::EnvAccess<Erc20Minable> for Erc20Minable {
        fn caller() -> <Erc20Minable as metis_lang::Env>::AccountId {
            Self::env().caller()
        }
        fn transferred_balance() -> <Erc20Minable as metis_lang::Env>::Balance {
            Self::env().transferred_balance()
        }
    }
    #[cfg(not(feature = "ink-as-dependency"))]
    const _: () = {
        use erc20;
        impl metis_lang::Storage<Erc20Minable, erc20::Data<Erc20Minable>> for Erc20Minable {
            fn get(&self) -> &erc20::Data<Erc20Minable> {
                &self.erc20
            }
            fn get_mut(&mut self) -> &mut erc20::Data<Erc20Minable> {
                &mut self.erc20
            }
        }
    };
    #[cfg(not(feature = "ink-as-dependency"))]
    const _: () = {
        impl erc20::EventEmit<Erc20Minable> for Erc20Minable {
            fn emit_event_transfer(
                &mut self,
                from: Option<AccountId>,
                to: Option<AccountId>,
                value: Balance,
            ) {
                self.env().emit_event(Transfer { from, to, value });
            }
            fn emit_event_approval(
                &mut self,
                owner: AccountId,
                spender: AccountId,
                value: Balance,
            ) {
                self.env().emit_event(Approval {
                    owner,
                    spender,
                    value,
                });
            }
        }
    };
}
