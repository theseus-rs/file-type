use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27355579: FileFormat = FileFormat {
    id: 27_355_579,
    source_type: SourceType::Wikidata,
    name: "ADRG Overview Image File",
    extensions: &["ovr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
