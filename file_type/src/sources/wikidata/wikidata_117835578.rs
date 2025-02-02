use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117835578: FileFormat = FileFormat {
    id: 117_835_578,
    source_type: SourceType::Wikidata,
    name: "DataBeam file",
    extensions: &["dbx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
