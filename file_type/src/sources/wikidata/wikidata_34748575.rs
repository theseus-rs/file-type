use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_34748575: FileFormat = FileFormat {
    id: 34_748_575,
    source_type: SourceType::Wikidata,
    name: "Thermo-Calc Database Format",
    extensions: &["tdb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
