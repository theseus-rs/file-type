use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27121524: FileFormat = FileFormat {
    id: 27_121_524,
    puid: "wikidata/27121524",
    name: "fixed width text file",
    extensions: &["fwf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
