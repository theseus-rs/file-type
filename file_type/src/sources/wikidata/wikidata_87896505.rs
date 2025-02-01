use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_87896505: FileFormat = FileFormat {
    id: 87_896_505,
    puid: "wikidata/87896505",
    name: "Avery DesignPro Document 4",
    extensions: &["zdp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
