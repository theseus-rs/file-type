use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205736: FileFormat = FileFormat {
    id: 28_205_736,
    puid: "wikidata/28205736",
    name: "Award BIOS logo, version 2",
    extensions: &["bmp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
