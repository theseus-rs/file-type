use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131850479: FileFormat = FileFormat {
    id: 131_850_479,
    puid: "wikidata/131850479",
    name: "OpenVDB file format",
    extensions: &["vdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
