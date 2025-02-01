use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_6026738: FileFormat = FileFormat {
    id: 6_026_738,
    puid: "wikidata/6026738",
    name: "PFB",
    extensions: &["pfb"],
    media_types: &["font/x-postscript-pfb"],
    internal_signatures: &[],
    related_formats: &[],
};
