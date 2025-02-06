use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27350170: FileFormat = FileFormat {
    id: 27_350_170,
    source_type: SourceType::Wikidata,
    name: "ADRG Transmittal Header File",
    extensions: &["thf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
