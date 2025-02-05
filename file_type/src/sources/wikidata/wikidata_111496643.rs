use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111496643: FileFormat = FileFormat {
    id: 111_496_643,
    source_type: SourceType::Wikidata,
    name: "Spectrum 512 Extended, version 2",
    extensions: &["spx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
