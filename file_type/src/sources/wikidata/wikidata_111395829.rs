use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111395829: FileFormat = FileFormat {
    id: 111_395_829,
    puid: "wikidata/111395829",
    name: "PhotoSuite Project File",
    extensions: &["pzp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
