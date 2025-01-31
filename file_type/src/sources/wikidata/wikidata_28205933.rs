use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205933: FileFormat = FileFormat {
    id: 28_205_933,
    puid: "wikidata/28205933",
    name: "Doodle! uncompressed image",
    extensions: &["dd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
