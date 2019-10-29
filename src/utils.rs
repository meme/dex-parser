use crate::encoded_item::EncodedItem;
use crate::encoded_item::EncodedItemArray;
use crate::jtype::Type;
use crate::uint;
use crate::ushort;

macro_rules! try_gread_vec_with {
    ($source:ident,$offset:ident,$cap:expr,$ctx:expr) => {{
        let cap = $cap as usize;
        let ctx = $ctx;
        let mut vec = Vec::with_capacity(cap);
        // NOTE: gread_inout_with doesn't work when de-serializing encoded array
        // so using an explicit loop here.
        for _ in 0..cap {
            vec.push($source.gread_with($offset, ctx)?);
        }
        vec
    }};
}

macro_rules! encoded_array {
    ($source:ident,$dex:ident,$offset:ident,$size:expr) => {
        if $size > 0 {
            let encoded_array_ctx = EncodedItemArrayCtx::new($dex, $size as usize);
            Some($source.gread_with($offset, encoded_array_ctx)?)
        } else {
            None
        }
    };
}

pub(crate) fn from_item<T, F, U>(
    array: Option<EncodedItemArray<T>>,
    f: F,
) -> Option<super::Result<Vec<U>>>
where
    F: Fn(T) -> super::Result<U>,
    T: EncodedItem,
{
    array.map(|array| array.into_iter().map(f).collect())
}

macro_rules! try_from_item {
    ($array:expr,$closure:expr) => {{
        use crate::utils::from_item;
        match from_item($array, $closure) {
            Some(v) => Some(v?),
            None => None,
        }
    }};
}

pub(crate) fn get_types<S>(dex: &super::Dex<S>, type_ids: &[ushort]) -> super::Result<Vec<Type>>
where
    S: AsRef<[u8]>,
{
    type_ids
        .into_iter()
        .map(|type_id| dex.get_type(uint::from(*type_id)))
        .collect()
}