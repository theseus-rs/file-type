use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_61964300: FileFormat = FileFormat {
    id: 61_964_300,
    source_type: SourceType::Wikidata,
    name: "GSSI SIR-10 RADAN data file",
    extensions: &["dzt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
