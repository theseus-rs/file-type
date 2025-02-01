use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127265031: FileFormat = FileFormat {
    id: 127_265_031,
    puid: "wikidata/127265031",
    name: "ANSYS input file",
    extensions: &["cdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
