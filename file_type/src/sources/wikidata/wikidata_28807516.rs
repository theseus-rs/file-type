use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28807516: FileFormat = FileFormat {
    id: 28_807_516,
    puid: "wikidata/28807516",
    name: "Microsoft Office Binder File for Windows 95",
    extensions: &["obd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
