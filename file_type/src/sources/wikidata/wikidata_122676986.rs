use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122676986: FileFormat = FileFormat {
    id: 122_676_986,
    puid: "wikidata/122676986",
    name: "CMX Corel Clipart",
    extensions: &["cmx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
