use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27473615: FileFormat = FileFormat {
    id: 27_473_615,
    source_type: SourceType::Wikidata,
    name: "Band Interleaved by Line Image File",
    extensions: &["bil"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
