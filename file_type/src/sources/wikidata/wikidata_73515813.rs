use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_73515813: FileFormat = FileFormat {
    id: 73_515_813,
    source_type: SourceType::Wikidata,
    name: "Palantir WinTime Plan",
    extensions: &["pln"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
