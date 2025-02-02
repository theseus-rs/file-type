use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27350170: FileFormat = FileFormat {
    id: 27_350_170,
    source_type: SourceType::Wikidata,
    name: "ADRG Transmittal Header File",
    extensions: &["thf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
