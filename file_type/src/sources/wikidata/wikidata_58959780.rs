use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_58959780: FileFormat = FileFormat {
    id: 58_959_780,
    puid: "wikidata/58959780",
    name: "Microsoft Excel Chart, version 2",
    extensions: &["xlc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
