use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113652622: FileFormat = FileFormat {
    id: 113_652_622,
    puid: "wikidata/113652622",
    name: "G3 1-D encoded FAX file format",
    extensions: &["fax"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
