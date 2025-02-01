use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_120984438: FileFormat = FileFormat {
    id: 120_984_438,
    puid: "wikidata/120984438",
    name: "Microsoft Publisher Template",
    extensions: &["pub"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
