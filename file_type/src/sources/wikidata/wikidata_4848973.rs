use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_4848973: FileFormat = FileFormat {
    id: 4_848_973,
    source_type: SourceType::Wikidata,
    name: "Bak file",
    extensions: &["bak"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
