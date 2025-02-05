use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205979: FileFormat = FileFormat {
    id: 28_205_979,
    source_type: SourceType::Wikidata,
    name: "Digital Video Interactive I Color Channel",
    extensions: &["imi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
