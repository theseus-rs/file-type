use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66458674: FileFormat = FileFormat {
    id: 66_458_674,
    puid: "wikidata/66458674",
    name: "Adobe Dimensions file format",
    extensions: &["dim"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
