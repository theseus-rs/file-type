use crate::format::FileFormat;

pub(crate) const LINGUIST_302: FileFormat = FileFormat {
    id: 302,
    puid: "linguist/302",
    name: "PureScript",
    extensions: &["purs"],
    media_types: &["text/x-haskell"],
    internal_signatures: &[],
    related_formats: &[],
};
