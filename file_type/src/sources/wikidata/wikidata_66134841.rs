use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66134841: FileFormat = FileFormat {
    id: 66_134_841,
    puid: "wikidata/66134841",
    name: "ACCDA file format",
    extensions: &["accda"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
