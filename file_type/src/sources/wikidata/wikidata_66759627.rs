use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66759627: FileFormat = FileFormat {
    id: 66_759_627,
    puid: "wikidata/66759627",
    name: "Space-delimited formatted text file",
    extensions: &["prn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
