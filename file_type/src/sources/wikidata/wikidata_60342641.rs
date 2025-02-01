use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_60342641: FileFormat = FileFormat {
    id: 60_342_641,
    puid: "wikidata/60342641",
    name: "Microsoft Excel Macro-Enabled Template",
    extensions: &["xltm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
