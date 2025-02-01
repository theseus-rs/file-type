use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206811: FileFormat = FileFormat {
    id: 28_206_811,
    puid: "wikidata/28206811",
    name: "Paint.NET image",
    extensions: &["pdn"],
    media_types: &["image/x-paintnet"],
    internal_signatures: &[],
    related_formats: &[],
};
