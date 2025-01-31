use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111395879: FileFormat = FileFormat {
    id: 111_395_879,
    puid: "wikidata/111395879",
    name: "FloppyShot Image",
    extensions: &["pic"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
