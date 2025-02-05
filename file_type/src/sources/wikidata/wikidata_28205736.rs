use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205736: FileFormat = FileFormat {
    id: 28_205_736,
    source_type: SourceType::Wikidata,
    name: "Award BIOS logo, version 2",
    extensions: &["bmp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
