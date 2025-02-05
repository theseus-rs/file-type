use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_21620033: FileFormat = FileFormat {
    id: 21_620_033,
    source_type: SourceType::Wikidata,
    name: "XDMF",
    extensions: &["xdmf", "xmf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
