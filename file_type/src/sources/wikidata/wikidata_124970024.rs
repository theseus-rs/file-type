use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_124970024: FileFormat = FileFormat {
    id: 124_970_024,
    source_type: SourceType::Wikidata,
    name: "MIX metadata file",
    extensions: &["mixmeta"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
