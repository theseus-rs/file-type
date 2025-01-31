use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_3596392: FileFormat = FileFormat {
    id: 3_596_392,
    puid: "wikidata/3596392",
    name: "Microsoft Word document template",
    extensions: &["dot", "dot"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
