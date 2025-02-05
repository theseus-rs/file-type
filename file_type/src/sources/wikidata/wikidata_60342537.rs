use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_60342537: FileFormat = FileFormat {
    id: 60_342_537,
    source_type: SourceType::Wikidata,
    name: "SmartDraw format",
    extensions: &["sdr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
