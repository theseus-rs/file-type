use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_63165558: FileFormat = FileFormat {
    id: 63_165_558,
    source_type: SourceType::Wikidata,
    name: "Microsoft Office Binder Template for Windows, version 97-2003",
    extensions: &["obt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
