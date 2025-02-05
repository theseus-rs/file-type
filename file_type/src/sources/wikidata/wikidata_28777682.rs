use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28777682: FileFormat = FileFormat {
    id: 28_777_682,
    source_type: SourceType::Wikidata,
    name: "mm",
    extensions: &["mm"],
    media_types: &["application/freemind", "application/x-freemind"],
    signatures: &[],
    related_formats: &[],
};
