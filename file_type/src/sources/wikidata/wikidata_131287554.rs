use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131287554: FileFormat = FileFormat {
    id: 131_287_554,
    puid: "wikidata/131287554",
    name: "tcsh script file format",
    extensions: &["csh", "tcsh"],
    media_types: &["application/x-csh", "application/x-csh"],
    internal_signatures: &[],
    related_formats: &[],
};
