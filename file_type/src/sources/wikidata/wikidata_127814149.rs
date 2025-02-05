use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127814149: FileFormat = FileFormat {
    id: 127_814_149,
    source_type: SourceType::Wikidata,
    name: "Ox source code file",
    extensions: &["ox"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
