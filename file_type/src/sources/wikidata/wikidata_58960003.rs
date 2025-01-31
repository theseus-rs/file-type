use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_58960003: FileFormat = FileFormat {
    id: 58_960_003,
    puid: "wikidata/58960003",
    name: "Microsoft Excel Chart, version 3",
    extensions: &["slc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
