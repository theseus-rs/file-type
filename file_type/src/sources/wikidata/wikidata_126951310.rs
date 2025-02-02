use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_126951310: FileFormat = FileFormat {
    id: 126_951_310,
    source_type: SourceType::Wikidata,
    name: "Haskell Script File Format",
    extensions: &["hs"],
    media_types: &["text/x-haskell"],
    internal_signatures: &[],
    related_formats: &[],
};
