use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47524799: FileFormat = FileFormat {
    id: 47_524_799,
    source_type: SourceType::Wikidata,
    name: "Quark Xpress Data File, version 9",
    extensions: &["qcd", "qct", "qpt", "qxp"],
    media_types: &["application/vnd.Quark.QuarkXPress"],
    internal_signatures: &[],
    related_formats: &[],
};
