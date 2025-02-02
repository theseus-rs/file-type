use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131192582: FileFormat = FileFormat {
    id: 131_192_582,
    source_type: SourceType::Wikidata,
    name: "Uxntal source code file",
    extensions: &["tal"],
    media_types: &["text/x-uxntal"],
    internal_signatures: &[],
    related_formats: &[],
};
