use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206053: FileFormat = FileFormat {
    id: 28_206_053,
    source_type: SourceType::Wikidata,
    name: "ERDAS LAN",
    extensions: &["lan"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
