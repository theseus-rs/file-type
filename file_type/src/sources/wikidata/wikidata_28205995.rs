use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205995: FileFormat = FileFormat {
    id: 28_205_995,
    source_type: SourceType::Wikidata,
    name: "Digital Video Interactive Color Map",
    extensions: &["imc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
