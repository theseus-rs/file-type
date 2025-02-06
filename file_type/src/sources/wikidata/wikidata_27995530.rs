use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27995530: FileFormat = FileFormat {
    id: 27_995_530,
    source_type: SourceType::Wikidata,
    name: "Electronic Arts BIG Archive",
    extensions: &["big", "viv"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
