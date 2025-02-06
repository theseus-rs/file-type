use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206508: FileFormat = FileFormat {
    id: 28_206_508,
    source_type: SourceType::Wikidata,
    name: "Light Sheet Microscope",
    extensions: &["lsm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
