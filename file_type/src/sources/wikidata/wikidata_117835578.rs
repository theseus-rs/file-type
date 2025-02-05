use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117835578: FileFormat = FileFormat {
    id: 117_835_578,
    source_type: SourceType::Wikidata,
    name: "DataBeam file",
    extensions: &["dbx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
