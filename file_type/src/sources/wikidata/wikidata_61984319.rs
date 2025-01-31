use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61984319: FileFormat = FileFormat {
    id: 61_984_319,
    puid: "wikidata/61984319",
    name: "Microsoft Visual FoxPro Class Library",
    extensions: &["vcx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
