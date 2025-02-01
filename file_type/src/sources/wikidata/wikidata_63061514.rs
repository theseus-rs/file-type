use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_63061514: FileFormat = FileFormat {
    id: 63_061_514,
    puid: "wikidata/63061514",
    name: "Microsoft Word Document, version 97-2003",
    extensions: &["doc", "doc", "wbk", "wbk"],
    media_types: &[
        "application/msword",
        "application/msword",
        "application/msword",
        "application/msword",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
