use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66660836: FileFormat = FileFormat {
    id: 66_660_836,
    puid: "wikidata/66660836",
    name: "Adobe Premiere Sequence",
    extensions: &["psq"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
