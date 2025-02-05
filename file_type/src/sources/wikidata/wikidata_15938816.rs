use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_15938816: FileFormat = FileFormat {
    id: 15_938_816,
    source_type: SourceType::Wikidata,
    name: "MATLAB M-File",
    extensions: &["m"],
    media_types: &["text/matlab"],
    signatures: &[],
    related_formats: &[],
};
