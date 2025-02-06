use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127514871: FileFormat = FileFormat {
    id: 127_514_871,
    source_type: SourceType::Wikidata,
    name: "Text2tags file",
    extensions: &["t2t"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
